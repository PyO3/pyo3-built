#!/bin/sh

set -e

cargo check

cd example
python setup.py build_rust -v --inplace
python setup.py test
cd $TRAVIS_BUILD_DIR
