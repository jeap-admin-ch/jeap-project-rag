/* Compatibility shims for glibc 2.38+ __isoc23_* symbols on older glibc.
 *
 * The prebuilt onnxruntime binary used by `ort-sys` (pulled in via fastembed)
 * is compiled against glibc 2.38+ and links to __isoc23_strtol/strtoll/strtoull.
 * On systems with older glibc (e.g. Ubuntu 22.04 with glibc 2.35) those
 * symbols don't exist; the C23 variants only differ from the originals in
 * how they treat the "0b" binary prefix, which is never produced by the
 * onnxruntime call sites that need these.
 */

#include <stdlib.h>

long __isoc23_strtol(const char *nptr, char **endptr, int base) {
    return strtol(nptr, endptr, base);
}

long long __isoc23_strtoll(const char *nptr, char **endptr, int base) {
    return strtoll(nptr, endptr, base);
}

unsigned long __isoc23_strtoul(const char *nptr, char **endptr, int base) {
    return strtoul(nptr, endptr, base);
}

unsigned long long __isoc23_strtoull(const char *nptr, char **endptr, int base) {
    return strtoull(nptr, endptr, base);
}
