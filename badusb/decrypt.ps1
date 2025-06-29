$filePath = "$env:USERPROFILE\Desktop\test_encrypted.hh2025"
$outputPath = "$env:USERPROFILE\Desktop\test_decrypted.txt"
$key = "<ключ_из_RANSOM_INFO.txt>"

if (Test-Path $filePath) {
  $combined = [System.IO.File]::ReadAllBytes($filePath)
  $iv = $combined[0..15]
  $encrypted = $combined[16..($combined.Length - 1)]
  $aes = [System.Security.Cryptography.Aes]::Create()
  $aes.KeySize = 256
  $aes.BlockSize = 128
  $aes.Mode = 'CBC'
  $aes.Key = [System.Text.Encoding]::UTF8.GetBytes($key)
  $aes.IV = $iv
  $decryptor = $aes.CreateDecryptor()
  $decrypted = $decryptor.TransformFinalBlock($encrypted, 0, $encrypted.Length)
  [System.Text.Encoding]::UTF8.GetString($decrypted) | Out-File -FilePath $outputPath -Encoding UTF8
  Write-Host "File decrypted to $outputPath"
} else {
  Write-Host "File not found!"
}
