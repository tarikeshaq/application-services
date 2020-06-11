#!/usr/bin/env python3

# Purpose: Run cargo update and make a pull-request against master.
# Dependencies: None
# Usage: ./automation/cargo-update-pr.py

import argparse
from datetime import datetime
import subprocess
import webbrowser

from shared import step_msg, fatal_err, run_cmd_checked, ensure_working_tree_clean

parser = argparse.ArgumentParser(description="Run cargo update and make a pull-request against master")
parser.add_argument("--remote",
                    default="origin",
                    help="The remote name that corresponds to the Application Services main repository.")

args = parser.parse_args()

step_msg("Running cargo update")
run_cmd_checked(["cargo", "update"])

step_msg("Regenerating dependency summaries")
res = subprocess.run(["./tools/regenerate_dependency_summaries.sh"])
if res.returncode != 0:
    fatal_err("Unable to complete dependency regeneration")
