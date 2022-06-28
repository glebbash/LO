FROM gitpod/workspace-full

USER gitpod

# install deno
RUN curl -fsSL https://deno.land/x/install/install.sh | sh
RUN /home/gitpod/.deno/bin/deno completions bash > /home/gitpod/.bashrc.d/90-deno \
  && echo 'export DENO_INSTALL="/home/gitpod/.deno"' >> /home/gitpod/.bashrc.d/90-deno \
  && echo 'export PATH="$DENO_INSTALL/bin:$PATH"' >> /home/gitpod/.bashrc.d/90-deno

# install llvm by installation script
RUN wget https://apt.llvm.org/llvm.sh
RUN sudo chmod +x llvm.sh && sudo ./llvm.sh 14
