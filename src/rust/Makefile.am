# Cargo binding

EXTRA_DIST = Cargo.toml \
    src/actions_process_list_provides.rs \
    src/actions_process.rs \
    src/autoconf.rs.in \
    src/cfg/mod.rs \
    src/clap_actions.rs \
    src/clap_fern.rs \
    src/cli_clap.rs \
    src/db/db_enviroment.rs \
    src/db/db_fs_dir.rs \
    src/db/db_fs_file.rs \
    src/db/db_job_depend.rs \
    src/db/db_job_provide.rs \
    src/db/db_job_require_variable_pair.rs \
    src/db/db_job_require_variable.rs \
    src/db/db_job.rs \
    src/db/db_provider.rs \
    src/db/db_session.rs \
    src/db/fs_dir_type.rs \
    src/db/mod.rs \
    src/db_variable_name.rs \
    src/db_variable_pair.rs \
    src/elephant.rs \
    src/jobs_load.rs \
    src/json_loader_elephant.rs \
    src/listener.rs \
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

DISTCLEANFILES = \
	$(desktop_files) \
	$(null)

check:
	cargo check
	cargo test
