import ImageView from "@/componenets/image_view";
import {useEffect, useState} from "react";
import assert from "assert";
import useSWR from "swr";
// @ts-ignore
import {get} from '@andreekeberg/imagedata';

// @ts-ignore
const fetcher = (...args) => fetch(...args).then(res => res.blob());

export default function ImageViewer() {
    const {data, error, isLoading} = useSWR("https://clideo.com/files/content/twitter-meme-maker-1.png", fetcher)


    const [image, setImage] = useState<ImageData | undefined>(undefined);
    useEffect(() => {
        if (data === undefined) {
            return;
        }

        // @ts-ignore
        get(data, (err, image) => {
            if (err) {
                console.error(err);
            } else {
                setImage(image);
            }
        });


    }, [data, error]);

    if (isLoading) {
        return <div>Loading...</div>
    } else { return <div>
        <ImageView image={image}/>
    </div>

    }
}
