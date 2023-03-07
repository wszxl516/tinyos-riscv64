#include "printf.h"
#include "uart.h"

static inline void set(char *buf, u32 size, usize *out, char val)
{
	if (*out < size - 1)
		buf[*out] = val;
	else if (*out == size - 1)
		buf[*out] = '\0';
	*out = *out + 1;
}

/**
 * This macro assigns a value to the buffer at the given index. It increments
 * the index after assigning the value. However, it takes care of bounds checks
 * so that we don't have to constantly think about them. It also "automatically"
 * nul-terminates the string if we hit the end of the buffer.
 */
#define SET(buf, size, out, val) set(buf, size, &out, val)

/**
 * This function implements the %x format specifier.
 */
static inline usize _format_hex(char *buf, u32 size, usize out,
                                   usize val, bool lz)
{
	usize mask = 0xf000000000000000;
	usize shift = 64;
	usize digit;
	char c;

	do {
		shift -= 4;
		digit = (val & mask) >> shift;

		if (digit || lz || shift == 0) {
			lz = true;
			c = (digit >= 10 ? 'a' + digit - 10 : '0' + digit);
			SET(buf, size, out, c);
		}
		mask >>= 4;
	} while (shift > 0);
	return out;
}

static inline usize _format_mac(char *buf, u32 size, usize out, usize mac)
{
	for (u32 i = 0; i < 6; i++) {
		if (i > 0)
			SET(buf, size, out, ':');
		out = _format_hex(buf, size, out, (usize)(mac >> (8 * i) & 0xff) , false);
	}
	return out;
}

/**
 * Implements the %u and %d format specifiers.
 */
static inline usize _format_int(char *buf, u32 size, usize out,
                                   usize val, bool is_signed)
{
	u8 tmp[20]; // max base 10 digits for 32-bit int
	usize tmpIdx = 0, rem = 0;
	if (is_signed && (val & (0x8000000000000000))) {
		val = ~(val) + 1;
		SET(buf, size, out, '-');
	}

	do {
		rem = val % 10; // should do uidivmod, only one call
		val = val / 10;
		tmp[tmpIdx++] = rem;
	} while (val);
	do {
		tmpIdx--;
		SET(buf, size, out, '0' + tmp[tmpIdx]);
	} while (tmpIdx > 0);
	return out;
}

static inline usize _format_float(char *buf, u32 size, usize out, double val)
{
	usize int_bits = (usize)val;
	double unit = 100;
	usize float_bits = (usize)(val * unit - (double)int_bits * unit);
	out = _format_int(buf, size, out, int_bits, false);
	SET(buf, size, out, '.');
	return _format_int(buf, size, out, float_bits, false);
}

/**
 * Implements the %I format specifier - IPv4 address, in network byte order.
 *
 */
static inline usize _format_ipv4(char *buf, u32 size, usize out,
                                    usize val)
{
	out = _format_int(buf, size, out, (val >> 0) & 0xFF, false);
	SET(buf, size, out, '.');
	out = _format_int(buf, size, out, (val >> 8) & 0xFF, false);
	SET(buf, size, out, '.');
	out = _format_int(buf, size, out, (val >> 16) & 0xFF, false);
	SET(buf, size, out, '.');
	out = _format_int(buf, size, out, (val >> 24) & 0xFF, false);
	return out;
}

/**
 * Implements the %s format specifier.
 */
static inline u32 _format_str(char *buf, u32 size, usize out,
                                   char *val)
{
	for (; *val; val++) {
		SET(buf, size, out, *val);
	}
	return out;
}

/**
 * This is the fundamental formatting function, although it is not the one users
 * will call frequently. The v means that it takes a va_list directly, which is
 * useful for sharing code across variadic functions. The s means that it will
 * write to a buffer. The n means that it will not write past the given buffer
 * size.
 *
 * Supports a minimal subset of standard C format language. Format specifiers
 * may only be a single character: field widths, padding bytes, etc, may not be
 * specified in this implementation. Available format specifiers: %x, %s
 *
 * @buf: Where to write the output
 * @size: Size of the output buffer
 * @format: Format string
 * @vl: Argument list
 * @return: number of bytes written
 */
usize vsnprintf(char *buf, u32 size, const char *format, va_list vl)
{
	usize out = 0;
	usize uintval=0;
	double float_val=0.0;
	char *strval = NULL;
	char charval = 0;

	for (u16 in = 0; format[in]; in++) {
		if (format[in] == '%') {
			in++;

			// when string ends with %, copy it literally
			if (!format[in]) {
				SET(buf, size, out, '%');
				goto nul_ret;
			}

			// otherwise, handle format specifiers
			switch (format[in]) {
			case 'x':
				uintval = va_arg(vl, usize);
				out = _format_hex(buf, size, out, uintval, false);
				break;
			case '0':
				if (format[in+1] != 'x') {
					SET(buf, size, out, '%');
					SET(buf, size, out, '0');
				} else {
					uintval = va_arg(vl, usize);
					out = _format_hex(buf, size, out, uintval, true);
					in++;
				}
				break;
			case 's':
				strval = va_arg(vl, char *);
				out = _format_str(buf, size, out, strval);
				break;
			case 'u':
			case 'd':
				uintval = va_arg(vl, usize);
				out = _format_int(buf, size, out, uintval,
				                  format[in] == 'd');
				break;
			case 'F':
			case 'f':
				float_val = va_arg(vl, double);
				out = _format_float(buf, size, out, float_val);
				break;
			case 'I':
				uintval = va_arg(vl, usize);
				out = _format_ipv4(buf, size, out, uintval);
				break;
			case 'M':
				uintval = va_arg(vl, usize);
				out = _format_mac(buf, size, out, uintval);
				break;
			case 'c':
				charval = (char)(0xFF & va_arg(vl, int));
				SET(buf, size, out, charval);
				break;
			case '%':
				SET(buf, size, out, '%');
				break;
			default:
				// default is to copy the unrecognized specifier
				// that may not be a great idea...
				SET(buf, size, out, '%');
				SET(buf, size, out, format[in]);
			}

		} else {
			SET(buf, size, out, format[in]);
		}
	}
nul_ret:
	SET(buf, size, out, '\0');
	return out - 1; // final count does not include nul-terminator
}

/**
 * Format a string into a buffer, without exceeding its size. See vsnprintf()
 * for details on formatting.
 * @buf: Where to write the output
 * @size: Size of the output buffer
 * @format: Format string
 * @return: Number of bytes written
 */
usize snprintf(char *buf, u32 size, const char *format, ...)
{
	u32 res;
	va_list vl;
	va_start(vl, format);
	res = vsnprintf(buf, size, format, vl);
	va_end(vl);
	return res;
}

/**
 * Format a string and print it to the console. See vsnprintf() for details on
 * formatting.
 *
 * NOTE: There is a fixed buffer size (see below). Make sure your messages will
 * fit into it. Also, the buffer is stack-allocated, so we need to be careful
 * with the size, or we may start running into the TAGS section.
 *
 * @format: Format string
 * @return: Number of bytes written
 */
usize printf(const char *format, ...)
{
	char buf[1024], *str;
	usize res = 0;
	va_list vl;
	va_start(vl, format);
	res = vsnprintf(buf, sizeof(buf), format, vl);
	va_end(vl);
	str = buf;
	while(*str != '\0') {
		put_c(*str);
		str++;
	}
	return res;
}

void print_bits(usize size, void *ptr)
{
    u8 *b = (u8*) ptr;
    u8 byte;
    int i, j;
    for (i = size-1; i >= 0; i--) {
        for (j = 7; j >= 0; j--) {
            byte = (b[i] >> j) & 1;
            printf("%u", byte);
        }
    }
}

int atoi(const char *str)
{
	int i = 0, val = 0;
	bool negate = false;
	if (str[i] == '-') {
		negate = true;
		i++;
	}
	for (; str[i]; i++)
		val = val * 10 + (str[i] - '0');
	return negate ? -val : val;
}