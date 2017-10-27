# Cargo binding

EXTRA_DIST = Cargo.toml \
    src/cli_clap.rs \
    src/dbFsDirType.rs \
    src/dbJobProvide.rs \
    src/dbJob.rs \
    src/dbSession.rs \
    src/elephant.rs \
    src/mdlJobModule.rs \
    src/dbEnviroment.rs \
    src/dbFsFile.rs \
    src/dbJobRequireVariablePair.rs \
    src/dbProvider.rs \
    src/dbVariableName.rs \
    src/jobs_load.rs \
    src/loader.rs \
    src/dbFsDir.rs \
    src/dbJobDepend.rs \
    src/dbJobRequireVariable.rs \
    src/db.rs \
    src/dbVariablePair.rs \
    src/json_loader_elephant.rs \
    src/main.rs \
	$(null)

EXTRA_PROGRAMS = target/@CARGO_TARGET_DIR@/whenenv

# we need this dummy target for make dist
target_@CARGO_TARGET_DIR@_whenenv_SOURCES =

all-local:
	cargo build --release

clean-local:
	-cargo clean

install-exec-local:
	$(MKDIR_P) $(DESTDIR)$(bindir)
	$(INSTALL) -c -m 755 target/@CARGO_TARGET_DIR@/whenenv $(DESTDIR)$(bindir)

DISTCLEANFILES =                                \
	$(desktop_files)                        \
	$(null)


check:
	cargo check
	cargo test
