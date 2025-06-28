# Inspired from https://mohundro.com/blog/2008-04-24-powershell-script-to-print-diff-output-in-color
Process {
    if ($_) {
        foreach ($line in $_) {
            if ($line -match '^[<|-]') {
                Write-Host -ForegroundColor red $line
            }
            elseif ($line -match '^[>|+]') {
                Write-Host -ForegroundColor green $line
            }
            elseif ($line -match '^[@]') {
                Write-Host -ForegroundColor blue $line
            }
            else {
                Write-Host $line
            }
        }
    }
}
