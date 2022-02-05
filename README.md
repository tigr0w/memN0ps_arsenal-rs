# MimiRust - Hacking the Windows operating system to hand us the keys to the kingdom with Rust.
<p>MimiRust is a post-exploitation tool that can be used within redteam operations. Like the name suggests the entire project is made within the Rust language. MimiRust is capable of the following actions:</p>
<ul>
  <li>Spawning any process as SYSTEM</li>
  <li>Extracting Windows passwords out of memory through the wdigest attack vector.</li>
  <li>Extracting Windows NTLM hashes from user accounts (aes / des)</li>
</ul><br>
<p>Todo:</p>
<ul>
  <li>Extracting Windows NTLM hashes from user accounts (md5 / rc4)</li>
</ul>

<h2>Which operating systems does MimiRust support</h2>
<p>It supports the following operating systems:</p>
<ul>
  <li>Win7 x64</li>
  <li>Windows Server 2008 x64</li>
  <li>Windows Server 2008R2 x64</li>
  <li>Win8 x64</li>
  <li>Windows Server 2012 x64</li>
  <li>Windows Server 2012R2 x64</li>
  <li>Win10_1507(and before 1903) x64</li>
</ul>
<small>Note: out of some tests it turned out that it does not dump credentials with some of the above operating systems. (however on other installations of the same operating system it did.)</small><br>
<br>
<h2>Why was MimiRust made</h2>
<p>I was bored in my first year of my CyberSecurity bachelors, as there wasn't anything cyber related being taught. Thus I decided I was going to start my own project, I already knew of Mimikatz for a while and why it was used however I did not know how it did it's thing, so to get this knowledge I decided to start learning how it does it's thing and thus mimiRust was created.</p>
<br>
<h2>Future plans</h2>
<p>For the future I want to add more functionality/capability to MimiRust (like getting NTLM hashes) so it can be fully employed inside of redteam operations.</p>
<br>
<h2>Demo</h2>
<iframe src="https://player.vimeo.com/video/673938805?h=e7b5ea87a7" width="640" height="360" frameborder="0" allow="autoplay; fullscreen; picture-in-picture" allowfullscreen></iframe>
