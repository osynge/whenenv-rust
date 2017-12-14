# Cargo binding

EXTRA_DIST = Cargo.toml \
    src/actions_process_list_provides.rs \
    src/actions_process.rs \
    src/clap_actions.rs \
    src/cli_clap.rs \
    src/dbEnviroment.rs \
    src/dbFsDir.rs \
    src/dbFsDirType.rs \
    src/dbFsFile.rs \
    src/dbJobDepend.rs \
    src/dbJobProvide.rs \
    src/dbJobRequireVariablePair.rs \
    src/dbJobRequireVariable.rs \
    src/dbJob.rs \
    src/dbProvider.rs \
    src/db.rs \
    src/dbSession.rs \
    src/dbVariableName.rs \
    src/dbVariablePair.rs \
    src/elephant.rs \
    src/jobs_load.rs \
    src/json_loader_elephant.rs \
    src/loader.rs \
    src/main.rs \
    src/mdlJobModule.rs \
	$(null)

EXTRA_PROGRAMS = target/@CARGO_TARGET_DIR@/whenenv

# we need this dummy target for make dist
target_@CARGO_TARGET_DIR@_whenenv_SOURCES =


all-local:
	cargo build @CARGO_RELEASE_ARGS@

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
