def calculate_wrapping_paper(length, width, height):
	top_area = length * width
	front_area = length * height
	side_area = width * height
	return 2 * (top_area + front_area + side_area) + min(top_area, front_area, side_area)

def parse_input(line):
	return [int(num) for num in line.split('x')]


total_wrapping_paper = 0

with open('./input') as f:
	dimensions = f.readlines()
	for dimension in dimensions:
		length, width, height = parse_input(dimension.strip())
		total_wrapping_paper += calculate_wrapping_paper(length, width, height)
print(total_wrapping_paper)	
