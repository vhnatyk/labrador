#!/bin/bash
export NVTX_ENABLE=1

# Run tests with output capture
{ cargo test --release binary_r1cs -- --skip test_completeness --nocapture && cargo test --release test::test_soundness -- --skip r1cs --nocapture && cargo test --release test::test_completeness -- --skip r1cs --nocapture; } |& tee all_tests.log
sed -i 's/\W\[[0-9]\+m//g' all_tests.log

# Run tests with nsys profiling
{ nsys profile --stats=true --force-overwrite true -o binary_r1cs_profile cargo test --release binary_r1cs -- --skip test_completeness --nocapture && \
  nsys profile --stats=true --force-overwrite true -o soundness_profile cargo test --release test::test_soundness -- --skip r1cs --nocapture && \
  nsys profile --stats=true --force-overwrite true -o completeness_profile cargo test --release test::test_completeness -- --skip r1cs --nocapture; } |& tee all_tests_nvtx.log

# Clean up raw NVTX symbols in log files again after nsys profiling
sed -i 's/\W\[[0-9]\+m//g' all_tests_nvtx.log