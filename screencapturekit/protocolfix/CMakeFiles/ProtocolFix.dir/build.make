# CMAKE generated file: DO NOT EDIT!
# Generated by "Unix Makefiles" Generator, CMake Version 3.25

# Delete rule output on recipe failure.
.DELETE_ON_ERROR:

#=============================================================================
# Special targets provided by cmake.

# Disable implicit rules so canonical targets will work.
.SUFFIXES:

# Disable VCS-based implicit rules.
% : %,v

# Disable VCS-based implicit rules.
% : RCS/%

# Disable VCS-based implicit rules.
% : RCS/%,v

# Disable VCS-based implicit rules.
% : SCCS/s.%

# Disable VCS-based implicit rules.
% : s.%

.SUFFIXES: .hpux_make_needs_suffix_list

# Command-line flag to silence nested $(MAKE).
$(VERBOSE)MAKESILENT = -s

#Suppress display of executed commands.
$(VERBOSE).SILENT:

# A target that is always out of date.
cmake_force:
.PHONY : cmake_force

#=============================================================================
# Set environment variables for the build.

# The shell in which to execute make rules.
SHELL = /bin/sh

# The CMake executable.
CMAKE_COMMAND = /usr/local/Cellar/cmake/3.25.2/bin/cmake

# The command to remove a file.
RM = /usr/local/Cellar/cmake/3.25.2/bin/cmake -E rm -f

# Escaping for special characters.
EQUALS = =

# The top-level source directory on which CMake was run.
CMAKE_SOURCE_DIR = /Users/pejo/projects/squad.fish/screencapturekit/protocolfix

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = /Users/pejo/projects/squad.fish/screencapturekit/protocolfix

# Include any dependencies generated for this target.
include CMakeFiles/ProtocolFix.dir/depend.make
# Include any dependencies generated by the compiler for this target.
include CMakeFiles/ProtocolFix.dir/compiler_depend.make

# Include the progress variables for this target.
include CMakeFiles/ProtocolFix.dir/progress.make

# Include the compile flags for this target's objects.
include CMakeFiles/ProtocolFix.dir/flags.make

CMakeFiles/ProtocolFix.dir/ProtocolFix.m.o: CMakeFiles/ProtocolFix.dir/flags.make
CMakeFiles/ProtocolFix.dir/ProtocolFix.m.o: ProtocolFix.m
CMakeFiles/ProtocolFix.dir/ProtocolFix.m.o: CMakeFiles/ProtocolFix.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/Users/pejo/projects/squad.fish/screencapturekit/protocolfix/CMakeFiles --progress-num=$(CMAKE_PROGRESS_1) "Building C object CMakeFiles/ProtocolFix.dir/ProtocolFix.m.o"
	/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -MD -MT CMakeFiles/ProtocolFix.dir/ProtocolFix.m.o -MF CMakeFiles/ProtocolFix.dir/ProtocolFix.m.o.d -o CMakeFiles/ProtocolFix.dir/ProtocolFix.m.o -c /Users/pejo/projects/squad.fish/screencapturekit/protocolfix/ProtocolFix.m

CMakeFiles/ProtocolFix.dir/ProtocolFix.m.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing C source to CMakeFiles/ProtocolFix.dir/ProtocolFix.m.i"
	/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -E /Users/pejo/projects/squad.fish/screencapturekit/protocolfix/ProtocolFix.m > CMakeFiles/ProtocolFix.dir/ProtocolFix.m.i

CMakeFiles/ProtocolFix.dir/ProtocolFix.m.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling C source to assembly CMakeFiles/ProtocolFix.dir/ProtocolFix.m.s"
	/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -S /Users/pejo/projects/squad.fish/screencapturekit/protocolfix/ProtocolFix.m -o CMakeFiles/ProtocolFix.dir/ProtocolFix.m.s

# Object files for target ProtocolFix
ProtocolFix_OBJECTS = \
"CMakeFiles/ProtocolFix.dir/ProtocolFix.m.o"

# External object files for target ProtocolFix
ProtocolFix_EXTERNAL_OBJECTS =

libProtocolFix.a: CMakeFiles/ProtocolFix.dir/ProtocolFix.m.o
libProtocolFix.a: CMakeFiles/ProtocolFix.dir/build.make
libProtocolFix.a: CMakeFiles/ProtocolFix.dir/link.txt
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --bold --progress-dir=/Users/pejo/projects/squad.fish/screencapturekit/protocolfix/CMakeFiles --progress-num=$(CMAKE_PROGRESS_2) "Linking C static library libProtocolFix.a"
	$(CMAKE_COMMAND) -P CMakeFiles/ProtocolFix.dir/cmake_clean_target.cmake
	$(CMAKE_COMMAND) -E cmake_link_script CMakeFiles/ProtocolFix.dir/link.txt --verbose=$(VERBOSE)

# Rule to build all files generated by this target.
CMakeFiles/ProtocolFix.dir/build: libProtocolFix.a
.PHONY : CMakeFiles/ProtocolFix.dir/build

CMakeFiles/ProtocolFix.dir/clean:
	$(CMAKE_COMMAND) -P CMakeFiles/ProtocolFix.dir/cmake_clean.cmake
.PHONY : CMakeFiles/ProtocolFix.dir/clean

CMakeFiles/ProtocolFix.dir/depend:
	cd /Users/pejo/projects/squad.fish/screencapturekit/protocolfix && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /Users/pejo/projects/squad.fish/screencapturekit/protocolfix /Users/pejo/projects/squad.fish/screencapturekit/protocolfix /Users/pejo/projects/squad.fish/screencapturekit/protocolfix /Users/pejo/projects/squad.fish/screencapturekit/protocolfix /Users/pejo/projects/squad.fish/screencapturekit/protocolfix/CMakeFiles/ProtocolFix.dir/DependInfo.cmake --color=$(COLOR)
.PHONY : CMakeFiles/ProtocolFix.dir/depend

