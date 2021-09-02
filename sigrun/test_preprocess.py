import re

assert_funcs = """
func exit(code: int)
func write(fd: int, buf: *byte, count: int)

func asserti_(actual: int, expected: int, s: *byte, len: int) {
  if actual != expected {
    write(0, "[FAILED]: ", 10)
    write(0, s, len)
    write(0, "\\n", 1)
    exit(1)
  }
}

func assertb_(actual: bool, expected: bool, s: *byte, len: int) {
  if actual != expected {
    write(0, "[FAILED]: ", 10)
    write(0, s, len)
    write(0, "\\n", 1)
    exit(1)
  }
}
"""

with open("test.vd", "r") as f1, open("tmp.vd", "w") as f2:
  lines = f1.readlines()
  f2.write(assert_funcs)
  for (i, line) in enumerate(lines):
    matches = re.findall("asserti\((.*), (.*)\)$", line)
    for match in matches:
      s = "{}: {}".format(i + 1, match[0]) 
      line = line.replace('asserti({}, {})'.format(match[0], match[1]), 'asserti_({}, {}, "{}", {})'.format(match[0], match[1], re.escape(s), len(s)))

    matches = re.findall("assertb\((.*), (.*)\)$", line)
    for match in matches:
      s = "{}: {}".format(i + 1, match[0]) 
      line = line.replace('assertb({}, {})'.format(match[0], match[1]), 'assertb_({}, {}, "{}", {})'.format(match[0], match[1], re.escape(s), len(s)))
    
    f2.write(line)
