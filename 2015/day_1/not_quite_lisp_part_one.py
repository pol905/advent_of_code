def find_floor(instructions):
	current_floor = []
	patterns = { "(": ")", ")": "(" }
	for instruction in instructions:
		if current_floor and current_floor[-1] == patterns[instruction]:
			current_floor.pop()
		else:
			current_floor.append(instruction)
	if not current_floor:
		return 0
	final_floor = len(current_floor)
	return final_floor if current_floor[-1] == "(" else -final_floor

with open("input") as f:
	lines = f.readlines()
	print(find_floor(lines[0].strip()))

