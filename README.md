# remote-backup

This command line tool scans all the folders of the <folder> directory, 
finds the most recent one, and copy it over scp to the backup folder <backup>.

remote-backup -f "/home/dcrespe/config/simple-backup/output" -u "dcrespe@10.42.2.17:/home/dcrespe"

On the synology NAS, the backup folder is

    denis@192.168.0.141:/volume1/home/denis/backups/PC_LOCAL_BACKUP
  