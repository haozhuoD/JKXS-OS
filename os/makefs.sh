FAT32_DIR="../fat32-fuse"
FS_IMG="${FAT32_DIR}/fs.img"

mkdir -p ${FAT32_DIR}
if test ! -e ${FAT32_DIR}/${FS_IMG}
then
    dd if=/dev/zero of=${FAT32_DIR}/${FS_IMG} bs=1k count=512k
fi

sudo chmod 777 ${FS_IMG}
sudo umount ${FS_IMG}
sudo umount ${FS_IMG}
sudo mkfs.vfat -F 32 ${FS_IMG}

if test -e ${FAT32_DIR}/fs
then 
    sudo rm -rf ${FAT32_DIR}/fs
    sudo mkdir ${FAT32_DIR}/fs
else
    sudo mkdir ${FAT32_DIR}/fs
fi

sudo mount ${FS_IMG} ${FAT32_DIR}/fs
sudo rm -f ${FAT32_DIR}/fs/*

for programname in $(ls ../user/src/bin)
do
    # if [ $programname != "initproc.rs" ] && [ $programname != "user_shell.rs" ]
    # then 
    sudo cp ../user/target/riscv64gc-unknown-none-elf/release/${programname%.rs} ../fat32-fuse/fs/${programname%.rs}
    # fi
done

for programname in $(ls ../user/riscv64)
do 
    sudo cp ../user/riscv64/$programname ${FAT32_DIR}/fs/"$programname"
done

# for programname in $(ls ../user/busybox_lua_testsuites)
# do 
#     sudo cp ../user/busybox_lua_testsuites/$programname ${FAT32_DIR}/fs/"$programname"
# done

sudo umount ${FAT32_DIR}/fs
