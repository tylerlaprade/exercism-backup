def convert(number):
    sound_dict = { 3:"Pling",5:"Plang",7:"Plong" }
    rain_sound = ""
    for num in sound_dict.keys():
        if number % num == 0:
            rain_sound += sound_dict[num]
            number /= num
    return str(number) if len(rain_sound) == 0 else rain_sound
