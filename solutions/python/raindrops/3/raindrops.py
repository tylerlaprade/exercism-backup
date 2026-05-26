def convert(number):
    sound_dict = { 3:"Pling",5:"Plang",7:"Plong" }
    rain_sound = ""
    for num, sound in sound_dict.items():
        if number % num == 0:
            rain_sound += sound
            number /= num
    return rain_sound if rain_sound else str(number)
