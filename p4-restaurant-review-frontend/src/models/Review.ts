import * as borsh from "@project-serum/borsh";

export class Review {
    title: string;
    rating: number;
    description: string;

    constructor(title: string, rating: number, description: string) {
        this.title = title;
        this.rating = rating;
        this.description = description;
    }

    borshInstructionSchema = borsh.struct([
        borsh.u8("variant"),
        borsh.str("title"),
        borsh.str("description"),
        borsh.u8("rating"),
    ]);

    static borshAccountSchema = borsh.struct([
        borsh.bool("initialized"),
        borsh.str("title"),
        borsh.str("description"),
        borsh.u8("rating"),
    ]);

    serialize(): Buffer {
        const buffer = Buffer.alloc(1000); // we defined the 1000 value in the contract as `account_len`
        this.borshInstructionSchema.encode({ ...this, variant: 0 }, buffer);
        return buffer.slice(0, this.borshInstructionSchema.getSpan(buffer));
    }

    static deserialize(buffer?: Buffer): Review | null {
        if (!buffer) {
            return null;
        }

        try {
            const { title, rating, description } =
                this.borshAccountSchema.decode(buffer);
            return new Review(title, rating, description);
        } catch (e) {
            console.log("Deserialization error:", e);
            console.log(buffer);
            return null;
        }
    }
}
