-include ../tools.mk

# This test builds a shared object, then an executable that links it as a native
# rust library (constrast to an rlib). The shared library and executable both
# are compiled with address sanitizer, and we assert that a fault in the dylib
# is correctly detected.

# Note: currently we have to add -lasan to link, because it's not automatically
# added in the -Z sanitizer=address process.

ifeq ($(TARGET),x86_64-unknown-linux-gnu)
all:
	$(RUSTC) -g -Z sanitizer=address --crate-type dylib --target $(TARGET) -lasan library.rs
	$(RUSTC) -g -Z sanitizer=address --crate-type bin --target $(TARGET) -lasan -llibrary program.rs
	echo $(RUSTC)
	LD_LIBRARY_PATH=$(TMPDIR) $(TMPDIR)/program 2>&1 | grep -q stack-buffer-overflow
else
all:
endif
