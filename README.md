<h2>Project status</h2>
For the most part it works, but there are definitely improvements that could be made.<br>

<h2>What is this project about</h2>
PAR is short for Pixiv Artist Reviewer.<br>
It is a small gui app that allows a user to view and interact (follow, unfollow, bookmark images, etc.) with the artists that the user publically follows on <a href="https://www.pixiv.net/en/">pixiv</a>.<br>
In particular, this app can:<br>
Display an artist's profile picture, name, upload date of the latest bookmarked illustration by the user, amount of uploaded illustrations in the last 6 months.<br>
Display an artist's 4 most recently uploaded illustrations that include some information about themselves.<br>
An ability to bookmark those illustrations, as well as an ability to unfollow an artist.<br>
And a little bit more.<br>

<h2>Why?</h2>
I wanted to write something in Rust and this was the first interesting project idea that came to my mind.<br>
This project is also my first experience with GitHub.<br>

<h2>Requirements</h2>
See Cargo.toml for Rust dependencies.<br>
To use pixiv's api, PAR requires pixiv's refresh token. To get the token, you can use either <a href="https://gist.github.com/ZipFile/c9ebedb224406f4f11845ab700124362">this</a> or <a href="https://gist.github.com/upbit/6edda27cb1644e94183291109b8a5fde">this</a>.<br>
The app uses Python with <a href="https://pypi.org/project/PixivPy3/">PixivPy3</a> and <a href="https://pypi.org/project/pytz/">pytz</a> modules.<br>
PAR uses Windows' cmd to call Python. Because of that it is Windows only, although it should not be hard to change that and make the app cross-platform.<br>

<h2>Major todos</h2>
I think it is quite obvious that using Python is the biggest bottleneck for this app. I was hoping that there would be a Rust crate that would replace Python's PixivPy module, but all the pixiv crates I tested either didn't work or didn't have all the functionality that I wanted. In the end, I decided to give up and just use Python (hence the name of the script). So the first major todo would be to basically get rid of Python. To do that, I will probably just translate parts that i need from PixivPy to Rust.<br>
Another todo would be making all the download operations async to avoid ui locking.<br>
