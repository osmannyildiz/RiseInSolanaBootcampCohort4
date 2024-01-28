import * as borsh from "@project-serum/borsh";

export class Review {
    title: string;
    description: string;
    rating: number;
    location: string;

    constructor(title: string, description: string, rating: number, location: string) {
        this.title = title;
        this.description = description;
        this.rating = rating;
        this.location = location;
    }

    borshInstructionSchema = borsh.struct([
        borsh.u8("variant"),
        borsh.str("title"),
        borsh.str("description"),
        borsh.u8("rating"),
        borsh.str("location"),
    ]);

    static borshAccountSchema = borsh.struct([
        borsh.bool("initialized"),
        borsh.str("title"),
        borsh.str("description"),
        borsh.u8("rating"),
        borsh.str("location"),
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
            const { title, description, rating, location } =
                this.borshAccountSchema.decode(buffer);
            return new Review(title, description, rating, location);
        } catch (e) {
            console.log("Deserialization error:", e);
            console.log(buffer);
            return null;
        }
    }
}
