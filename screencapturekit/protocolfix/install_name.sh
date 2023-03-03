#!/bin/bash
echo $1
if ! xcrun otool -l $1 | grep LC_RPATH;
then
   xcrun install_name_tool -add_rpath "@executable_path/Frameworks;@loader_path/Frameworks" $1;
else
    echo 'Otool Operation Skipped';

fi;
