def calculate_wrapping_paper(length, width, height):
	top_area = length * width
	front_area = length * height
	side_area = width * height
	return 2 * (top_area + front_area + side_area) + min(top_area, front_area, side_area)

def calculate_ribbon(length, width, height):
	min_side = min(length, width, height)
	if min_side == length:
		second_min_side = min(width, height)
	elif min_side == width:
		second_min_side = min(length, height)
	else:
		second_min_side = min(width, length)
	wrapping_ribbon = 2 * (min_side + second_min_side)
	bow_ribbon = length * width * height
	return wrapping_ribbon + bow_ribbon

def parse_input(line):
	return [int(num) for num in line.split('x')]


total_wrapping_paper = 0
total_ribbon_required = 0
with open('./input') as f:
	dimensions = f.readlines()
	for dimension in dimensions:
		length, width, height = parse_input(dimension.strip())
		total_wrapping_paper += calculate_wrapping_paper(length, width, height)
		total_ribbon_required += calculate_ribbon(length, width, height)
print(total_wrapping_paper, total_ribbon_required)	
