#!/usr/bin/env python3

import os
import sys

os.execvp("cargo", ["cargo", "run", "--bin"] + sys.argv[1:])
# os.execvp("cargo", ["cargo", "test"])