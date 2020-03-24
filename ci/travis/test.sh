#!/bin/sh

set -e

cargo check

cd example
python setup.py build_rust -v --inplace
python -c "import pprint, hello; pprint.pprint(hello.__build__)"
cd $TRAVIS_BUILD_DIR
