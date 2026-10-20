#pragma once
/* Integer-only freestanding build: the upstream headers declare debug helpers.
 * No stdio routine is linked or called by measured kernels. */
int printf(const char *, ...);
