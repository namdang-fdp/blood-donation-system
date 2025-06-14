import { Avatar, AvatarFallback } from './ui/avatar';

export interface AccountPictureProps {
    name: string;
}

export const AccountPicture = ({ name }: AccountPictureProps) => {
    return (
        <Avatar className="rounded-lg size-full">
            <AvatarFallback className="bg-rose-400/20 text-rose-500 text-[1em] rounded font-medium">
                {name[0]}
            </AvatarFallback>
        </Avatar>
    );
};
