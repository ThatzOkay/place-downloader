/*
 * Requirements:
 * the command 'ffmpeg' and 'montage' need to be present. Montage is provided by imagemagick.
 * A valid reddit place token.
 */

const framerate = 10
const frameEveryXSeconds = 30
const https=require('https')
const fs = require('fs');
const execSync = require('child_process').execSync;
const token='PUT YOUR TOKEN HERE! starts with ey.'
if(!token.startsWith('ey')) {
	console.log('no token present, exiting.')
	process.exit(1)
}

if(!fs.existsSync('frames')) fs.mkdirSync('frames');
if(!fs.existsSync('combined')) fs.mkdirSync('combined');
if(!fs.existsSync('tovideo')) fs.mkdirSync('tovideo');
const downloadFrame=(stamp,frame)=>new Promise(resolve=>{
	if(fs.existsSync("frames/"+stamp+'/'+frame.canvasIndex+'.png')) {
		resolve()
		return
	}
	const file = fs.createWriteStream("frames/"+stamp+'/'+frame.canvasIndex+'.png');
	const request = https.get(frame.url, function(response) {
		response.pipe(file);
		file.on("finish", () => {
			file.close();
			setTimeout(resolve,100)
		})
	});
})
downloadStamp=stamp=>new Promise(resolve=>{
	if(fs.existsSync('frames/'+stamp)) {
		resolve()
		return
	}
	const data={
		"operationName":"frameHistory",
		"variables":{
			"input":{
				"actionName":"get_frame_history",
				"GetFrameHistoryMessageData":{
					"timestamp":stamp
				}
			}
		},
		"query":`mutation frameHistory($input: ActInput!) {
			act(input: $input) {
				data {
					... on BasicMessage {
						id
						data {
							... on GetFrameHistoryResponseMessageData {
								frames {
									canvasIndex
									url
									__typename
								}
								__typename
							}
							__typename
						}
						__typename
					}
					__typename
				}
				__typename
			}
		}`.replaceAll('\t','')
	}
	const json=JSON.stringify(data)
	const request = https.request({
		hostname: 'gql-realtime-2.reddit.com',
		path: '/query',
		method: 'POST',
		headers: {
			'content-type': 'application/json',
			'authorization': 'Bearer '+token,
			'content-length': json.length
		}
	}, res=>{
		let data=''
		res.on('data',chunk=>data+=chunk)
		res.on('end',()=>{
			try {
				const parsed=JSON.parse(data)
				if(parsed.success===false) {
					console.log('ERROR DOWNLOADING JSON OF',stamp,data, 'If this is a unauthorized error, verify your token!')
					fs.appendFileSync('errors.txt', stamp+'\n');
				}
			} catch(e) {
				console.log('ERROR PARSING JSON FROM',stamp,'. Some frames are broken, but if this happens to all, you might have to fix your token!')
			}
			fs.mkdirSync('frames/'+stamp)
			fs.writeFileSync('frames/'+stamp+'/data.json',data)
			setTimeout(()=>resolve(),100)
		})
	})
	request.write(json)
	request.end()
})
function getTodo() {
	let todo=[]
	for(let stamp = 1689858000000; stamp <= 1690320892999+1000; stamp+=1000*frameEveryXSeconds) {
		todo.push(stamp)
	};
	todo=todo.reverse();
	return todo
}
(async () => {
	const stamps=getTodo()
	for (let [index, stamp] of stamps.entries()) {
		console.log(`started download json of ${stamp}, ${stamps.length-index} remaining`)
		await downloadStamp(stamp)
	}
	for (let [index, stamp] of stamps.entries()) {
		console.log(`started download frames of ${stamp}, ${stamps.length-index} remaining`)
		let data = fs.readFileSync(`frames/${stamp}/data.json`,'utf8')
		try {
			data=JSON.parse(data).data.act.data[0].data.frames
		} catch(e) {
			fs.appendFileSync('errors.txt', stamp+'\n');
			continue
		}
		await Promise.all(data.map(frame=>downloadFrame(stamp,frame)))
	}
	for (let [index, stamp] of stamps.entries()) {
		console.log(`started adding empty frames to ${stamp}, ${stamps.length-index} remaining`)
		for(let i=0;i<=5;i++) {
			if(!fs.existsSync(`frames/${stamp}/${i}.png`)) {
				fs.copyFileSync('frames/empty.png',`frames/${stamp}/${i}.png`)
			}
		}
	}
	for (let [index, stamp] of stamps.entries()) {
		console.log(`started merging frames of ${stamp}, ${stamps.length-index} remaining`)
		if(fs.existsSync(`combined/${stamp}.png`)) continue
		const output = execSync(`montage [0-7].png -tile 3x2 -geometry +0+0 ../../combined/${stamp}.png`, { encoding: 'utf-8', cwd: `frames/${stamp}` });
	}
	for (let [index, stamp] of stamps.entries()) {
		console.log(`started copying ${stamp}, ${stamps.length-index} remaining`)
		if(fs.existsSync(`tovideo/${stamp}.png`)) continue
		fs.copyFileSync(`combined/${stamp}.png`,`tovideo/${stamp}.png`)
	}
	console.log('started ffmpeg...')
	execSync(`ffmpeg -r ${framerate} -framerate ${framerate} -pattern_type glob -i '*.png' -c:v libx264 -pix_fmt yuv420p ${frameEveryXSeconds}min-${framerate}fps.mp4`, {cwd: 'tovideo'})
})()