ACLOCAL_AMFLAGS = -I m4
SUBDIRS = src
MAINTAINERCLEANFILES = Makefile.in configure config-h.in \
                        stamp-h.in

topdir=@RPM_DIR@
	mkdir -p $(topdir)/SOURCES \
	$(topdir)/SPECS \
	$(topdir)/BUILD \
	$(topdir)/RPMS $(topdir)/SRPMS
	cp @PACKAGE@-@VERSION@.tar.gz $(topdir)/SOURCES
	cp @PACKAGE@.spec $(topdir)/SPECS
	rpmbuild --define "_topdir $(topdir)" -ba @PACKAGE@.spec
	mkdir -p $(topdir)/DEBS
	rm -rf $(topdir)/DEBS/@PACKAGE@-@VERSION@.tar.gz
	cp @PACKAGE@-@VERSION@.tar.gz $(topdir)/DEBS/
	cd $(topdir)/DEBS  && tar -zxvf @PACKAGE@-@VERSION@.tar.gz
	cd $(topdir)/DEBS/@PACKAGE@-@VERSION@ && dpkg-buildpackage -rfakeroot

# There is something I don't fully understand in the next part
# Sometimes 'config' exist sometimes not, I have to check to be sure
# Maybe it's because I rerun the process without erasing the previous
# version ?
# Et Hop! Another tricky thing I didnt understood: depcomp exist on
# ccdevli05 but not on ccdesk05... Have fun !

# Looked into this and its strange but logical, and caused by autogen.sh
# not being distributed, I seem to remember... Have fun, also!


dist-hook:
	(cd $(distdir); \
	if [ -a $(ac_aux_dir) ]; then \
	   echo "config exist!"; \
	else \
	   mkdir $(ac_aux_dir); \
	fi)
	(if [ -a $(ac_aux_dir)/depcomp ]; then \
	   echo "depcomp exist!"; \
	else \
	   touch $(ac_aux_dir)/depcomp; \
	fi)
	for file in $(AUX_DIST) $(AUX_DIST_EXTRA); do \
	  cp $$file $(distdir)/$$file; \
	done




elekickerdocdir = ${prefix}/doc/whenenv
elekickerdoc_DATA = \
	COPYING\
	AUTHORS\
	ChangeLog\
	INSTALL

EXTRA_DIST =  $(elekickerdoc_DATA)



srpm: dist @PACKAGE@.spec
	mkdir -p $(topdir)/SOURCES \
	$(topdir)/SPECS \
	$(topdir)/BUILD \
	$(topdir)/RPMS $(topdir)/SRPMS
	cp @PACKAGE@-@VERSION@.tar.gz $(topdir)/SOURCES
	cp @PACKAGE@.spec $(topdir)/SPECS

rpm: srpm
	rpmbuild --define "_topdir $(topdir)" -ba @PACKAGE@.spec


