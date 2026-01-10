function round(x)
    return x + 0.5 - (x + 0.5) % 1
end

-- user@host
if context ~= nil then
	print(""
		..bold()
		..distroColors[1]
		..context.user
		..reset()
		..bold()
		.."@"
		..distroColors[2]
		..context.host
		..reset())
end

-- OS
if distro ~= nil then
	print(""
		..bold()
		..distroColors[2]
		.."OS"
		..reset()
		..": "
		..distro.short_name
		.." "
		..distro.architecture)
end

-- Host
if host ~= nil then
	print(""
		..bold()
		..distroColors[2]
		.."Host"
		..reset()
		..": "
		..host.model)
end

-- Kernel
if kernel ~= nil then
	print(""
		..bold()
		..distroColors[2]
		.."Kernel"
		..reset()
		..": "
		..kernel.name
		.." "
		..kernel.version)
end

-- Uptime
if uptime ~= nil then
	local output = ""
	local function comma()
		if output ~= "" then output = output..", " end
	end
	local function s(v)
		if v ~= 1 and v ~= 0 then
			return "s"
		else
			return ""
		end
	end
	if uptime.days >= 1 then
		output = output..uptime.days.." day"..s(uptime.days)
	end
	if uptime.hours >= 1 then
		comma()
		output = output..uptime.hours.." hour"..s(uptime.hours)
	end
	if uptime.minutes >= 1 then
		comma()
		output = output..uptime.minutes.." minute"..s(uptime.minutes)
	elseif uptime.hours == 0 then
		comma()
		output = output..uptime.seconds.." second"..s(uptime.seconds)
	end

	print(""
		..bold()
		..distroColors[2]
		.."Uptime"
		..reset()
		..": "
		..output)
end

-- Packages
if packageManagers ~= nil then
	local output = ""
	if #packageManagers ~= 0 then
		for i,packageManager in pairs(packageManagers) do
			if packageManager.packages == 0 then
				table.remove(packageManagers, i)
			end
		end
		for i,packageManager in pairs(packageManagers) do
			if i ~= #packageManagers then
				output = output
					..packageManager.packages
					.." ("
					..packageManager.name
					.."), "
			else
				output = output
					..packageManager.packages
					.." ("
					..packageManager.name
					..")"
			end
		end
	else
		output = "0"
	end
	print(""
		..bold()
		..distroColors[2]
		.."Packages"
		..reset()
		..": "
		..output)
end

-- Shell
if shell ~= nil then
	print(""
		..bold()
		..distroColors[2]
		.."Shell"
		..reset()
		..": "
		..shell.name
		.." "
		..shell.version)
end

-- Resolution
if resolution ~= nil then
	if resolution.refresh ~= nil then
		print(""
			..bold()
			..distroColors[2]
			.."Resolution"
			..reset()
			..": "
			..resolution.width
			.."x"
			..resolution.height
			.." @ "
			..round(resolution.refresh)
			.."Hz")
	else
		print(""
			..bold()
			..distroColors[2]
			.."Resolution"
			..reset()
			..": "
			..resolution.width
			.."x"
			..resolution.height)
	end
end

-- DE
if de ~= nil then
	print(""
		..bold()
		..distroColors[2]
		.."DE"
		..reset()
		..": "
		..de.name
		.." "
		..de.version)
end

-- WM
if wm ~= nil then
	print(""
		..bold()
		..distroColors[2]
		.."WM"
		..reset()
		..": "
		..wm)
end

-- CPU
if cpu ~= nil then
	local freq = (cpu.freq >= 1000)
		and ""..(cpu.freq / 1000).."GHz"
		or  ""..cpu.freq.."MHz"
	print(""
		..bold()
		..distroColors[2]
		.."CPU"
		..reset()
		..": "
		..cpu.name
		.." ("
		..cpu.cores
		..") @ "
		..freq)
end

-- CPU Temp (right after CPU)
if temperature ~= nil and temperature.cpu ~= nil then
	print(""
		..bold()
		..distroColors[2]
		.."CPU Temp"
		..reset()
		..": "
		..round(temperature.cpu)
		.."°C")
end

-- GPU
if gpus ~= nil then
	if #gpus ~= 1 then
		print(""
			..bold()
			..distroColors[2]
			.."GPUs"
			..reset()
			..": ")
		for _,gpu in pairs(gpus) do
			print(" - "..gpu.brand.." "..gpu.name)
		end
	else
		print(""
			..bold()
			..distroColors[2]
			.."GPU"
			..reset()
			..": "
			..gpus[1].brand
			.." "
			..gpus[1].name)
	end
end

-- GPU Temp (right after GPU)
if temperature ~= nil and temperature.gpu ~= nil then
	print(""
		..bold()
		..distroColors[2]
		.."GPU Temp"
		..reset()
		..": "
		..round(temperature.gpu)
		.."°C")
end

-- Monitors
if monitors ~= nil and monitors.count > 0 then
	if monitors.count == 1 then
		print(""
			..bold()
			..distroColors[2]
			.."Monitor"
			..reset()
			..": "
			..monitors.monitors[1].name)
	else
		print(""
			..bold()
			..distroColors[2]
			.."Monitors"
			..reset()
			..": ")
		for _,monitor in pairs(monitors.monitors) do
			print(" - "..monitor.name)
		end
	end
end

-- Motherboard
if motherboard ~= nil then
	print(""
		..bold()
		..distroColors[2]
		.."Board"
		..reset()
		..": "
		..motherboard.vendor
		.." "
		..motherboard.name)
end

-- Memory
if memory ~= nil then
	-- This memory math is probably inaccurate, but idk how to make it right ;-;
	print(""
		..bold()
		..distroColors[2]
		.."Memory"
		..reset()
		..": "
		..math.floor(memory.used / 1024)
		.."MB / "
		..math.floor(memory.max / 1024)
		.."MB")
end

-- Battery
if battery ~= nil then
	local output = battery.capacity.."% ("..battery.status..")"
	if battery.health ~= nil then
		output = output.." Health: "..battery.health.."%"
	end
	if battery.cycles ~= nil then
		output = output.." Cycles: "..battery.cycles
	end
	print(""
		..bold()
		..distroColors[2]
		.."Battery"
		..reset()
		..": "
		..output)
end

-- Disk
if disk ~= nil then
	print(""
		..bold()
		..distroColors[2]
		.."Disk ("
		..disk.mount_point
		..")"
		..reset()
		..": "
		..disk.used_gb
		.."GB / "
		..disk.total_gb
		.."GB")
end

-- Network
if network ~= nil then
	print(""
		..bold()
		..distroColors[2]
		.."Network"
		..reset()
		..": "
		..network.interface
		.." ("
		..network.ip
		..")")
end

-- Bluetooth
if bluetooth ~= nil and bluetooth.count > 0 then
	print(""
		..bold()
		..distroColors[2]
		.."Bluetooth"
		..reset()
		..": "
		..bluetooth.count
		.." devices")
end
print("")
print(""
	..blackBg()  .."   "
	..redBg()    .."   "
	..greenBg()  .."   "
	..yellowBg() .."   "
	..blueBg()   .."   "
	..magentaBg().."   "
	..cyanBg()   .."   "
	..whiteBg()  .."   "
	..reset())
print(""
	..blackBrightBg()  .."   "
	..redBrightBg()    .."   "
	..greenBrightBg()  .."   "
	..yellowBrightBg() .."   "
	..blueBrightBg()   .."   "
	..magentaBrightBg().."   "
	..cyanBrightBg()   .."   "
	..whiteBrightBg()  .."   "
	..reset())
