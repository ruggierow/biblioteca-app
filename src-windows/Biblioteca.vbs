Dim oShell, oFSO, sDir, sEdge, sPaths, i
Set oShell = CreateObject("WScript.Shell")
Set oFSO   = CreateObject("Scripting.FileSystemObject")
sDir = oFSO.GetParentFolderName(WScript.ScriptFullName)

oShell.Run "powershell.exe -WindowStyle Hidden -ExecutionPolicy Bypass -File """ & sDir & "\servidor.ps1"" """ & sDir & """ 49152", 0, False
WScript.Sleep 1200

sPaths = Array( _
  oShell.ExpandEnvironmentStrings("%ProgramFiles%\Microsoft\Edge\Application\msedge.exe"), _
  oShell.ExpandEnvironmentStrings("%ProgramFiles(x86)%\Microsoft\Edge\Application\msedge.exe"), _
  oShell.ExpandEnvironmentStrings("%LocalAppData%\Microsoft\Edge\Application\msedge.exe"))
sEdge = ""
For i = 0 To UBound(sPaths)
  If oFSO.FileExists(sPaths(i)) Then
    sEdge = sPaths(i)
    Exit For
  End If
Next

If sEdge <> "" Then
  oShell.Run """" & sEdge & """ --app=http://localhost:49152/ --profile-directory=Biblioteca --no-first-run --disable-notifications --disable-default-apps --disable-features=DialMediaRouteProvider"
Else
  oShell.Run "start https://ruggierow.github.io/biblioteca-app/"
End If
