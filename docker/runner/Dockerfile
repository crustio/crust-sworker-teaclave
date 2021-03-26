FROM baiduxlab/sgx-rust

ADD incubator-teaclave-sgx-sdk /opt/intel/incubator-teaclave-sgx-sdk
ADD src /crust-sworker/src
ADD scripts /crust-sworker/scripts
ADD VERSION /crust-sworker/VERSION
ADD buildenv.mk /crust-sworker/buildenv.mk
ADD Makefile /crust-sworker/Makefile
ADD docker/runner/start_sworker.sh /start_sworker.sh
RUN /crust-sworker/scripts/install.sh
CMD /start_sworker.sh
