param (
    [int]$NumberOfInstances = 3
)

function Get-FreePort {
    $listener = [System.Net.Sockets.TcpListener]::new('127.0.0.1', 0)

    try {
        $listener.Start()
        $port = $listener.Server.LocalEndPoint.Port
        return $port
    } catch {
        Write-Output "Error finding free port: $_"
    } finally {
        $listener.Stop()
    }
}

$allHostnames = @()

for ($i = 1; $i -le $NumberOfInstances; $i++) {
    $freePort = Get-FreePort
    $hostname = "localhost:$freePort"
    Start-Process -FilePath "cmd" -ArgumentList "/c .\target\debug\rust-short-url.exe --application-url $hostname -m in-memory" -WorkingDirectory "."
    Write-Output "Instance $i started on port $freePort"
    $allHostnames += $hostname
}

$freePort = Get-FreePort
$hostname = "localhost:$freePort"
Start-Process -FilePath "cmd" -ArgumentList "/c .\target\debug\rust-short-url.exe --application-url $hostname -m coordinator --hostnames `"$allHostnames`"" -WorkingDirectory "."
Write-Output "Coordinator started on port $freePort"