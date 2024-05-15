!define APP_NAME "CopyAsUniversalPath"

# define installer Name
OutFile "${APP_NAME}Setup.exe"

# set install directory
InstallDir "$PROFILE\${APP_NAME}"

# default section start
Section
	# define output path
	SetOutPath $INSTDIR

	# specify file to go in output path
	File "D:\source_control\Rust\to_universal_path\target\release\to_universal_path.exe"

	# define uninstaller Name
	WriteUninstaller $INSTDIR\uninstaller.exe
	
	# Modify the registery to add explorer context menu items, these will need to be undone for uninstall
	SetRegView 64
	; WriteRegStr HKCU \
		; "Software\Classes\*\shell\copy_as_universal_path\command" \
		; "" \
		; "$\"$PROFILE\${APP_NAME}\to_universal_path.exe$\" $\"%1$\""
		
	# Register for file
	WriteRegStr HKCU \
		"Software\Classes\*\shell\copy_as_universal_path" \
		"MUIVerb" \
		"Copy as Universal Path"
	WriteRegStr HKCU \
		"Software\Classes\*\shell\copy_as_universal_path" \
		"SubCommands" \
		"copy_as_universal_path;copy_as_universal_path_escape_and_quote"
	WriteRegStr HKCU \
		"Software\Classes\*\shell\copy_as_universal_path" \
		"Extended" \
		""
	# Register for folder
	WriteRegStr HKCU \
		"Software\Classes\directory\shell\copy_as_universal_path" \
		"MUIVerb" \
		"Copy as Universal Path"
	WriteRegStr HKCU \
		"Software\Classes\directory\shell\copy_as_universal_path" \
		"SubCommands" \
		"copy_as_universal_path;copy_as_universal_path_escape_and_quote"
	WriteRegStr HKCU \
		"Software\Classes\directory\shell\copy_as_universal_path" \
		"Extended" \
		""
		
	# Register the actual commands
	WriteRegStr HKLM \
		"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\CommandStore\shell\copy_as_universal_path" \
		"MUIVerb" \
		"Copy as Universal Path"
	WriteRegStr HKLM \
		"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\CommandStore\shell\copy_as_universal_path\command" \
		"" \
		"$\"$PROFILE\${APP_NAME}\to_universal_path.exe$\" $\"%1$\""
	WriteRegStr HKLM \
		"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\CommandStore\shell\copy_as_universal_path_escape_and_quote" \
		"MUIVerb" \
		"Copy as Universal Path (quote and escape)"
	WriteRegStr HKLM \
		"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\CommandStore\shell\copy_as_universal_path_escape_and_quote\command" \
		"" \
		"$\"$PROFILE\${APP_NAME}\to_universal_path.exe$\" $\"%1$\" --mode=EscapeAndQuote"

	; WriteRegStr HKLM "SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\CommandStore\shell\copy_as_universal_path\command" "command" "$\"D:\source_control\Rust\to_universal_path\target\release\to_universal_path.exe$\" $\"%1$\" --mode=EscapeAndQuote"
SectionEnd # Default section end

# create a section to define what the uninstaller does.
# the section will always be named "Uninstall"
Section "Uninstall"
	SetRegView 64
	# delete file context menu item
	DeleteRegKey HKCU "Software\Classes\*\shell\copy_as_universal_path"
	# delete folder context menu item
	DeleteRegKey HKCU "Software\Classes\directory\shell\copy_as_universal_path"
	
	# Delete the actual commands
	DeleteRegKey HKLM "SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\CommandStore\shell\copy_as_universal_path_escape_and_quote"
	DeleteRegKey HKLM "SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\CommandStore\shell\copy_as_universal_path"
	
	# Delete installed file
	Delete $INSTDIR\to_universal_path.exe

	# Delete the uninstaller
	Delete $INSTDIR\uninstaller.exe

	# Delete the directory
	RMDir $INSTDIR
SectionEnd # Uninstaller section end