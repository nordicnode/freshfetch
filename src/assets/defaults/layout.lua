print(""..save())

print("  "..string.gsub(art, "\n", "\n  "))

print(""..up(artHeight))

if not logo then
	print(""
		..right(artWidth + 4)
		..string.gsub(info,
			"\n",
			"\n"..right(artWidth + 4)))
end

print(""..restore())

if infoHeight > artHeight then
	print(""..down(infoHeight))
else
	print(""..down(artHeight))
end

