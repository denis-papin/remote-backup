# remote-backup

This command line tool scans all the folders of the <folder> directory, 
finds the most recent one, and copy it over scp to the backup folder <backup>.

remote-backup -f "/home/dcrespe/config/simple-backup/output" -u "dcrespe@10.42.2.17:/home/dcrespe"
  