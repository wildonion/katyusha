



class CPU:
	def __init__(self):
		pass

	def burn(self):
		while(True):
			number = 0
			if(number >= sys.maxsize):
				number = 0
			else:
				number = number + 1