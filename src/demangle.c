#include <demangle.h>

char* cplus_demangle_wrapper(const char* mangled_name, int show_params, int show_ansi) {
  // This is a simple wrapper solely to allow using the demangle macros
  // DMGL_PARAMS and DMGL_ANSI.
  int options = DMGL_NO_OPTS;
  if (show_params) {
    options |= DMGL_PARAMS;
  }
  if (show_ansi) {
    options |= DMGL_ANSI;
  }
  return cplus_demangle(mangled_name, options);
}
