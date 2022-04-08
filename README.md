# ptarmigan

Ponderomotive trajectories and radiation emission

## Build

All of ptarmigan's default dependencies are Rust crates, which are downloaded automatically by Cargo. Building the code in this case is as simple as running:

```bash
cargo build --release [-j NUM_THREADS]
```

where `NUM_THREADS` is the number of separate threads that Cargo is allowed to spawn.

The following optional features are available:

* `with-mpi`, which enables parallel processing via MPI. Requires an MPI library (ptarmigan is tested against OpenMPI, versions <= 3.1, and MPICH) and the Clang compiler.
* `hdf5-output`, which enables output of complete particle data as an HDF5 file. Requires [libhdf5](https://www.hdfgroup.org/solutions/hdf5/).

To build with a combination of these features, run:

```bash
cargo build --release --features with-mpi,hdf5-output [-j NUM_THREADS]
```

The ptarmigan changelog can be found [here](docs/changelog.md).

## Specify problem

ptarmigan takes as its single argument the path to a YAML file describing the input configuration. Output is automatically written to the same directory as this file. The inputs for some test problems can be found in [examples](examples). Starting from scratch, the input needs to contain the following sections:

* control
* laser
* beam

and optionally

* constants
* output
* stats

The structure of the input file is described in detail [here](docs/input.md).

## Run

Assuming ptarmigan has been downloaded to `ptarmigan` and already built,

```bash
cd ptarmigan
[mpirun -n np] ./target/release/ptarmigan path/to/input.yaml
```

will run the code, parallelized over `np` MPI tasks (if MPI support has been enabled).

## Output

The code bins the final-state particles to generate the distribution functions requested in the input file, which are written in plain-text or FITS format.

If `hdf5-output` is enabled, complete data about all particles can be written as a single HDF5 file.
