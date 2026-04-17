; OVERCLOCK NSIS Installer Template
; Based on Tauri default template with custom Shortcut UX

!include "MUI2.nsh"
!include "nsDialogs.nsh"
!include "LogicLib.nsh"
!include "x64.nsh"

; ─── Variables ──────────────────────────────────────────────────────────────
Var Dialog
Var CheckboxDesktop
Var CheckboxStartMenu
Var CreateDesktopShortcut
Var CreateStartMenuShortcut

; ─── Branding ───────────────────────────────────────────────────────────────
Name "{{product_name}}"
OutFile "{{out_file}}"
InstallDir "$PROGRAMFILES64\{{product_name}}"
InstallDirRegKey HKLM "Software\{{manufacturer}}\{{product_name}}" "Install_Dir"

; ─── Interface Settings ─────────────────────────────────────────────────────
!define MUI_ABORTWARNING
!define MUI_ICON "..\..\..\..\icons\icon.ico"
!define MUI_UNICON "..\..\..\..\icons\icon.ico"
; !define MUI_HEADERIMAGE
; !define MUI_HEADERIMAGE_BITMAP "{{header_image}}"
; !define MUI_WELCOMEFINISHPAGE_BITMAP "{{sidebar_image}}"
!define MUI_BGCOLOR 07080C
!define MUI_TEXTCOLOR E2E4E9

; ─── Pages ──────────────────────────────────────────────────────────────────
!insertmacro MUI_PAGE_WELCOME
Page custom PageShortcuts PageShortcutsLeave
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES

!define MUI_FINISHPAGE_RUN "$INSTDIR\{{main_binary_name}}.exe"
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES

; ─── Languages ──────────────────────────────────────────────────────────────
!insertmacro MUI_LANGUAGE "English"

; ─── Functions ──────────────────────────────────────────────────────────────
Function .onInit
    ${If} ${RunningX64}
        # Success
    ${Else}
        MessageBox MB_OK "This application requires a 64-bit operating system."
        Abort
    ${EndIf}
    
    ; Default to checked
    StrCpy $CreateDesktopShortcut ${BST_CHECKED}
    StrCpy $CreateStartMenuShortcut ${BST_CHECKED}
FunctionEnd

Function PageShortcuts
    !insertmacro MUI_HEADER_TEXT "[*] INJECTION_VECTORS" "Select optional payload delivery vectors."
    
    nsDialogs::Create 1018
    Pop $Dialog
    ${If} $Dialog == error
        Abort
    ${EndIf}

    ${NSD_CreateLabel} 0 0 100% 12u "[+] INITIALIZING SHORTCUT HOOKS FOR OVERCLOCK v0.1..."
    Pop $0

    ${NSD_CreateCheckbox} 10u 25u 100% 12u "Inject Desktop.lnk (Quick Launch Vector)"
    Pop $CheckboxDesktop
    ${NSD_SetState} $CheckboxDesktop $CreateDesktopShortcut

    ${NSD_CreateCheckbox} 10u 40u 100% 12u "Inject StartMenu.lnk (System Menu Vector)"
    Pop $CheckboxStartMenu
    ${NSD_SetState} $CheckboxStartMenu $CreateStartMenuShortcut

    nsDialogs::Show
FunctionEnd

Function PageShortcutsLeave
    ${NSD_GetState} $CheckboxDesktop $CreateDesktopShortcut
    ${NSD_GetState} $CheckboxStartMenu $CreateStartMenuShortcut
FunctionEnd

; ─── Sections ───────────────────────────────────────────────────────────────
Section "Install" SEC01
    SetOutPath "$INSTDIR"
    
    ; Handlebars will inject binaries and resources here
    {{#each binaries}}
    File "/oname={{this.[1]}}" "{{this.[0]}}"
    {{/each}}
    
    {{#each resources}}
    File "/oname={{this.[1]}}" "{{@key}}"
    {{/each}}

    ; Write registry for uninstaller
    WriteRegStr HKLM "Software\{{manufacturer}}\{{product_name}}" "Install_Dir" "$INSTDIR"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\{{product_name}}" "DisplayName" "{{product_name}}"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\{{product_name}}" "UninstallString" '"$INSTDIR\uninstall.exe"'
    WriteUninstaller "$INSTDIR\uninstall.exe"

    ; ─── Shortcuts Logic ────────────────────────────────────────────────────
    ${If} $CreateDesktopShortcut == ${BST_CHECKED}
        CreateShortCut "$DESKTOP\{{product_name}}.lnk" "$INSTDIR\{{main_binary_name}}.exe"
    ${EndIf}

    ${If} $CreateStartMenuShortcut == ${BST_CHECKED}
        CreateDirectory "$SMPROGRAMS\{{product_name}}"
        CreateShortCut "$SMPROGRAMS\{{product_name}}\{{product_name}}.lnk" "$INSTDIR\{{main_binary_name}}.exe"
    ${EndIf}
SectionEnd

Section "Uninstall"
    Delete "$INSTDIR\{{main_binary_name}}.exe"
    Delete "$INSTDIR\uninstall.exe"
    
    ; Cleanup shortcuts
    Delete "$DESKTOP\{{product_name}}.lnk"
    RMDir /r "$SMPROGRAMS\{{product_name}}"

    RMDir "$INSTDIR"
    DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\{{product_name}}"
    DeleteRegKey HKLM "Software\{{manufacturer}}\{{product_name}}"
SectionEnd
