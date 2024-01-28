import { AppBar } from "@/components/AppBar";
import ReviewCard from "@/components/ReviewCard";
import { useEffect, useState } from "react";
import { Review } from "@/models/Review";
import * as web3 from "@solana/web3.js";
import { fetchReviews } from "@/util/fetchReviews";
import { useWallet } from "@solana/wallet-adapter-react";
import ReviewForm from "@/components/Form";

//Replace with your own Program_id
const REVIEW_PROGRAM_ID = "4bxHsLuaDvpdwXyoiG2stu913TLxLNpANFCWXBCtQpvC";

export default function Home() {
    const [txid, setTxid] = useState("");
    const [reviews, setReviews] = useState<Review[]>([]);

    const [title, setTitle] = useState("");
    const [rating, setRating] = useState(0);
    const [description, setDescription] = useState("");

    useEffect(() => {
        const fetchAccounts = async () => {};
        fetchAccounts();
    }, []);

    const handleSubmit = () => {
        const review = new Review(title, rating, description);
        handleTransactionSubmit(review);
    };

    const handleTransactionSubmit = async (review: Review) => {};

    return (
        <main
            className={`flex min-h-screen flex-col items-center justify-between p-24 `}
        >
            <div className="z-10 max-w-5xl w-full items-center justify-between font-mono text-sm lg:flex">
                <AppBar />
            </div>

            <div className="after:absolute after:-z-20 after:h-[180px] after:w-[240px] after:translate-x-1/3 after:bg-gradient-conic after:from-sky-200 after:via-blue-200 after:blur-2xl after:content-[''] before:dark:bg-gradient-to-br before:dark:from-transparent before:dark:to-blue-700/10 after:dark:from-sky-900 after:dark:via-[#0141ff]/40 before:lg:h-[360px]">
                <ReviewForm
                    title={title}
                    description={description}
                    rating={rating}
                    setTitle={setTitle}
                    setDescription={setDescription}
                    setRating={setRating}
                    handleSubmit={handleSubmit}
                />
            </div>

            {txid && <div>{txid}</div>}

            <div className="mb-32 grid text-center lg:max-w-5xl lg:w-full lg:mb-0 lg:grid-cols-3 lg:text-left">
                {reviews &&
                    reviews.map((review) => {
                        return (
                            <ReviewCard key={review.title} review={review} />
                        );
                    })}
            </div>
        </main>
    );
}
