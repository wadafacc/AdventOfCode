l = []
count = 0

def match(c: list):
  # print("".join(c))
  if "".join(c) in ["XMAS", "SAMX"]:
    return True
  return False

def repl(c: list):
  # print(c)
  None

with open('./inputs.txt') as f:
  while line := f.readline().rstrip():
    print(line)
    l.append(list(line))


print()

for y in range(0,len(l)):
  for x in range(0, len(l[y])):
    try:
      # horizontal
      h = [l[y][x], l[y][x+1], l[y][x+2], l[y][x+3]]
      print("h  ", h)
      if match(h):
        print("MATCH H")
        count += 1
    except IndexError:
      pass

    try:
      # vertical
      v = [l[y][x], l[y+1][x], l[y+2][x], l[y+3][x]]
      print("v  ", v)
      if match(v):
        print("MATCH V")
        count += 1
    except IndexError:
      pass

    try:
      # diagonal
      d1 = [l[y][x], l[y+1][x+1], l[y+2][x+2], l[y+3][x+3]]
      print("d1 ", d1)
      if match(d1):
        print("MATCH D1")
        count += 1
    except IndexError:
      pass
    
    try:
      d2 = [l[y][x], l[y+1][x-1], l[y+2][x-2], l[y+3][x-3]]
      print("d2 ", d2)
      if match(d2):
        print("MATCH D2")
        count += 1
    except IndexError:
      pass
  
    print()
print(count)
