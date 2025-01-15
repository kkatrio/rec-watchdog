### Rec-watchdog

Check if camera recordings are working fine, otherwise send notification.

```
docker build -t recwatchdog .
docker run -it -d --rm --name recwatchdog -v /mnt/diskos/frigatestorage:/mnt/diskos/frigatestorage recwatchdog
```
