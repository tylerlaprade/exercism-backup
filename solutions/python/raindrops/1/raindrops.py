def convert(number):
    nums = [3,5,7]
    words = ["Pling","Plang","Plong"]
    rain_sound = ""
    for idx in range(len(nums)):
        if number % nums[idx] == 0:
            rain_sound += words[idx]
            number /= nums[idx]
    return str(number) if len(rain_sound) == 0 else rain_sound
